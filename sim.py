from abc import ABC, abstractmethod
import numpy as np
from time import sleep



class Screen:
    def __init__(self, width=80, height=40, charmap=np.array([' ', '@'], dtype='<U1'), delay=0.1):
        self.width = width
        self.height = height
        self.charmap = charmap
        self.icharmap = {c:i for i, c in enumerate(charmap)}
        self.screen = np.zeros((height, width), dtype=np.uint8)
        self.delay = delay

    def draw_at(self, c, x, y):
        """draws a character at a given x, y position in the terminal"""
        self.screen[y, x] = self.icharmap[c]

    def clear(self):
        """clears the terminal screen"""
        self.screen = np.zeros((self.height, self.width), dtype=np.uint8)

    def draw(self):
        """render the screen, and return the cursor to the top left"""
        #move cursor to top left
        print("\033[0;0H", end="")

        #map the screen to the character map, and convert to a string
        lines = [''.join(line) for line in self.charmap[self.screen]]
        txt = '\n'.join(lines)

        #print the text
        print(txt, end="")
        print("\033[0;0H", end="")

        if self.delay is not None:
            sleep(self.delay)



class Actor(ABC):
    @abstractmethod
    def step(self): ...
    def render(self, screen:Screen): ... #TBD what how this works...

class Person(Actor):
    def __init__(self):
        self.x = 10
        self.y = 10

    def step(self):
        #for now, just move in a random direction
        dx = np.random.randn()
        dy = np.random.randn()

        self.x += dx
        self.y += dy
        
        #clamp to area
        self.x = np.clip(self.x, 0, 80)
        self.y = np.clip(self.y, 0, 80)

    def render(self, screen:Screen):
        screen.draw_at("@", int(self.x), int(self.y))


if __name__ == "__main__":
    
    p = Person()
    screen = Screen(delay=0.1)


    while True:
        screen.clear()
        p.step()
        p.render(screen)
        screen.draw()
