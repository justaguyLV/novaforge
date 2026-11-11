# Demo game script, written in Python. Edit me!

class Follow(Behaviour):
    """Moves the object toward the touch point."""
    def update(self, dt):
        tx, ty, down = Input.touch()
        if down:
            x, y = self.game_object.position
            self.game_object.position = (x + (tx - 25 - x) * min(1.0, 10 * dt),
                                         y + (ty - 25 - y) * min(1.0, 10 * dt))


class Pulse(Behaviour):
    def start(self):
        self.t = Random.range(0, 6)

    def update(self, dt):
        self.t += dt * 3
        s = 40 + 14 * ((self.t % 2) - 1 if (self.t % 2) < 1 else 1 - (self.t % 2 - 1) - 0)
        self.game_object.set_size(s, s)


@on_start
def start():
    w, h = Screen.size()
    log("NovaForge demo started", w, h)
    for i in range(60):
        o = GameObject(Random.range(0, w - 50), Random.range(0, h - 50), 40, 40,
                       rgb(Random.range(80, 255), Random.range(80, 255), Random.range(80, 255)))
        o.set_velocity(Random.range(-300, 300), Random.range(-300, 300), bounce=True)
        o.add(Pulse())
    player = GameObject(w / 2, h / 2, 70, 70, 0xFFD400)
    player.add(Follow())
