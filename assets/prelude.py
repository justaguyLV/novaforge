# NovaForge Python API (Unity-style). Loaded before your game script.

class GameObject:
    def __init__(self, x=0, y=0, w=50, h=50, color=0xFFFFFF):
        self.id = _spawn(float(x), float(y), float(w), float(h), int(color))
        self.behaviours = []
        _objects.append(self)

    @property
    def position(self):
        return _get_pos(self.id)

    @position.setter
    def position(self, p):
        _set_pos(self.id, float(p[0]), float(p[1]))

    @property
    def color(self):
        return 0

    @color.setter
    def color(self, rgb):
        _set_color(self.id, int(rgb))

    def set_size(self, w, h):
        _set_size(self.id, float(w), float(h))

    def set_velocity(self, vx, vy, bounce=False):
        _set_vel(self.id, float(vx), float(vy))
        _set_bounce(self.id, bool(bounce))

    def add(self, behaviour):
        behaviour.game_object = self
        self.behaviours.append(behaviour)
        behaviour.start()
        return behaviour

    def destroy(self):
        _despawn(self.id)
        if self in _objects:
            _objects.remove(self)


class Behaviour:
    """Subclass and override start()/update(dt), like Unity's MonoBehaviour."""
    game_object = None

    def start(self):
        pass

    def update(self, dt):
        pass


class Input:
    @staticmethod
    def touch():
        return _touch()  # (x, y, is_down)


class Screen:
    @staticmethod
    def size():
        return _screen()


class Random:
    @staticmethod
    def value():
        return _rand()

    @staticmethod
    def range(a, b):
        return a + (b - a) * _rand()


def log(*args):
    _log(" ".join([str(a) for a in args]))


def rgb(r, g, b):
    return (int(r) << 16) | (int(g) << 8) | int(b)


_objects = []
_start_fn = None


def on_start(fn):
    global _start_fn
    _start_fn = fn
    return fn


def _engine_start():
    if _start_fn is not None:
        _start_fn()


def _engine_update(dt):
    for go in list(_objects):
        for b in list(go.behaviours):
            b.update(dt)
