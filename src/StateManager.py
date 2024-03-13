from collections import deque
from logging import info, warning
import sys
import pyglet
import Actor

from Config import *
from Event import *
from HxPx import Hx, Px
from LogId import LOGID
from Quickle import ENCODER, DECODER

SCENE = 'scene'
OVERLAY = 'overlay'
ACTION_BAR = 'action_bar'

STATE_PLAY       = 1 << 0
STATE_UI_OVERLAY = 1 << 1

class Impl(pyglet.event.EventDispatcher):
    def __init__(self, actor_factory):
        self.state = 0
        self.seq = -1
        self.registry = {}
        self.actor_factory = actor_factory

    # Impl tries everything, and sends back what is done
    def on_do(self, tid, evt, broadcast):
        self.dispatch_event("do_{}".format(evt.event), tid, evt)
        self.dispatch_event("on_send", tid, evt, self.seq, broadcast)
    def on_try(self, tid, evt, seq):
        self.seq = seq
        self.dispatch_event("try_{}".format(evt.event), tid, evt)

    def try_init_connection(self, tid, evt):
        evt.tid = tid
        self.dispatch_event("on_do", tid, evt, False)

    def try_load_scene(self, tid, evt):
        for i,it in list(self.registry[SCENE].tiles.items()):
            self.dispatch_event("on_do", tid, TileChangeEvent(i.state, it.state), False)
        for i,it in list(self.registry[SCENE].npcs.items()) + list(self.registry[SCENE].pcs.items()): 
            self.dispatch_event("on_do", tid, ActorLoadEvent(it.state), False)
        z = self.registry[SCENE].generator.elevation(Hx(0,0,0))
        actor = self.actor_factory.create(tid, "blank", Px(0,0,z))
        self.dispatch_event('on_do', tid, ActorLoadEvent(actor.state), True)

    def on_close(self):
        info("saving scene")
        pyglet.resource.file("default.0","wb").write(ENCODER.dumps(self.registry[SCENE].state))
        sys.exit(0)

    def begin(self):
        self.push_handlers(self.registry[SCENE])
        self.registry[SCENE].push_handlers(self)
        self.state |= STATE_PLAY

    def register(self, id, it):
        self.registry[id] = it

Impl.register_event_type('do_change_tile')
Impl.register_event_type('do_discover_tile')
Impl.register_event_type('do_init_connection')
Impl.register_event_type('do_load_actor')
Impl.register_event_type('do_load_scene')
Impl.register_event_type('do_move_actor')
Impl.register_event_type('do_select_overlay')
Impl.register_event_type('do_unload_actor')

Impl.register_event_type('on_broadcast')
Impl.register_event_type('on_close')
Impl.register_event_type('on_do')
Impl.register_event_type('on_send')
Impl.register_event_type('on_try')

Impl.register_event_type('try_change_tile')
Impl.register_event_type('try_discover_tile')
Impl.register_event_type('try_init_connection')
Impl.register_event_type('try_load_actor')
Impl.register_event_type('try_load_scene')
Impl.register_event_type('try_move_actor')
Impl.register_event_type('try_unload_actor')

class StateManager(Impl):
    def __init__(self, window, key_state_handler, actor_factory):
        super().__init__(actor_factory)
        self.window = window
        self.key_state_handler = key_state_handler
        self.tid = None
        self.evt_deque = deque()

    # Client sends everything it tries to server, and does everything it is told to
    def on_do(self, tid, evt, broadcast, seq=None): 
        if seq is not None and tid == self.tid:
            while self.evt_deque:
                i, it = self.evt_deque.popleft()
                if i == seq: break
                else: warning("{:} - skipping seq {}".format(LOGID.SKIP_SEQ , i))
            evt.dt = 0
            self.dispatch_event("do_{}".format(evt.event), tid, evt)
            for i,it in list(self.evt_deque):
                if isinstance(it,ActorMoveEvent): it.dt = 0
                self.dispatch_event("do_{}".format(it.event), tid, it)
        else: self.dispatch_event("do_{}".format(evt.event), tid, evt)

    def on_try(self, tid, evt, sync):
        seq = None
        if not sync: 
            self.seq += 1
            seq = self.seq
            self.evt_deque.append((seq, evt))
            self.dispatch_event("try_{}".format(evt.event), tid, evt)
        self.dispatch_event("on_send", tid, evt, seq)
    
    def do_init_connection(self, tid, evt): 
        self.tid = evt.tid
        self.dispatch_event('on_try', self.tid, SceneLoadEvent(), True)

    def on_close(self, *args):
        if(self.state & STATE_UI_OVERLAY):
            self.window.pop_handlers()
            self.window.push_handlers(self.key_state_handler)
            self.window.push_handlers(self.registry[ACTION_BAR])
            self.state = STATE_PLAY

    def on_overlay(self, *args):
        if(self.state & STATE_PLAY):
            self.window.pop_handlers()
            self.window.pop_handlers()
            self.window.push_handlers(self.registry[OVERLAY])
            self.state |= STATE_UI_OVERLAY
            self.dispatch_event("on_open",*args)

    def begin(self):
        super().begin()
        self.window.push_handlers(self)
        self.window.push_handlers(self.key_state_handler)
        self.window.push_handlers(self.registry[ACTION_BAR])
        self.push_handlers(self.registry[OVERLAY])
        self.registry[OVERLAY].push_handlers(self)
        self.registry[OVERLAY].push_handlers(self.registry[SCENE])    
        self.dispatch_event('on_try', None, ConnectionInitEvent(None), True)

StateManager.register_event_type('on_open')
