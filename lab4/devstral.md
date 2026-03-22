```python
"""
Project Report

Requirements Engineering:
- Functional Requirements:
  1. The game must allow a player to control a snake that moves around the screen.
  2. The game should increase the snake's length when it eats food.
  3. The game should end if the snake collides with itself or the walls.

- Non-Functional Requirements:
  1. The game must run smoothly on any terminal that supports curses.
  2. The code must be well-documented and easy to understand for new developers.

Deployment:
- Instructions to Run the Game:
  Ensure Python is installed, then execute `python snake_game.py`.
- Required Dependencies: No additional dependencies required beyond standard Python library.

Codebase Support:
- Potential Bugs Specific to Curses:
  1. Curses can sometimes have issues with terminal resizing. To avoid this, make sure to handle SIGWINCH signals and reset the curses environment properly.
  2. If the curses screen is not properly terminated (e.g., via Ctrl+C), it may leave the terminal in a strange state. Always ensure proper cleanup using `curses.endwin()` in a try-finally block or signal handlers.

Documentation:
- Controls:
  - Use arrow keys to control the snake's direction.
  - Press 'q' to quit the game.
"""

import curses
import random

class SnakeGame:
    def __init__(self, stdscr):
        self.stdscr = stdscr
        curses.curs_set(0)
        self.sh, self.sw = self.stdscr.getmaxyx()
        self.win = curses.newwin(self.sh, self.sw, 0, 0)
        self.win.keypad(1)
        self.win.timeout(100)

        self.snake_x = self.sw//4
        self.snake_y = self.sh//2
        self.snake = [
            [self.snake_y, self.snake_x],
            [self.snake_y, self.snake_x-1],
            [self.snake_y, self.snake_x-2]
        ]

        self.food = [self.sh//2, self.sw//2]
        self.win.addch(int(self.food[0]), int(self.food[1]), curses.ACS_PI)

        self.key = curses.KEY_RIGHT

    def main_loop(self):
        while True:
            next_key = self.win.getch()
            self.key = key if next_key == -1 else next_key

            if self.key == ord('q'):
                break

            new_head = [self.snake[0][0], self.snake[0][1]]

            if self.key == curses.KEY_DOWN:
                new_head[0] += 1
            if self.key == curses.KEY_UP:
                new_head[0] -= 1
            if self.key == curses.KEY_LEFT:
                new_head[1] -= 1
            if self.key == curses.KEY_RIGHT:
                new_head[1] += 1

            self.snake.insert(0, new_head)

            if (
                self.snake[0][0] in [0, self.sh] or
                self.snake[0][1] in [0, self.sw] or
                self.snake[0] in self.snake[1:]
            ):
                curses.endwin()
                quit()

            if self.snake[0] == self.food:
                self.food = None
                while self.food is None:
                    nf = [
                        random.randint(1, self.sh-1),
                        random.randint(1, self.sw-1)
                    ]
                    self.food = nf if nf not in self.snake else None
                self.win.addch(self.food[0], self.food[1], curses.ACS_PI)
            else:
                self.snake.pop()

            self.win.addch(self.snake[0][0], self.snake[0][1], curses.ACS_CKBOARD)

def main(stdscr):
    game = SnakeGame(stdscr)
    game.main_loop()

if __name__ == "__main__":
    try:
        curses.wrapper(main)
    except KeyboardInterrupt:
        curses.endwin()
        print("\nGame interrupted and terminal state restored.")

# Testing
import unittest

class TestSnakeLogic(unittest.TestCase):
    def test_snake_movement(self):
        # Mock a simple snake movement to the right
        initial_pos = [5, 5]
        direction = curses.KEY_RIGHT
        new_head = [initial_pos[0], initial_pos[1] + 1]

        if direction == curses.KEY_DOWN:
            new_head[0] += 1
        elif direction == curses.KEY_UP:
            new_head[0] -= 1
        elif direction == curses.KEY_LEFT:
            new_head[1] -= 1
        elif direction == curses.KEY_RIGHT:
            new_head[1] += 1

        self.assertEqual(new_head, [5, 6])

    def test_snake_collision_with_self(self):
        # Create a snake that collides with itself
        snake = [
            [5, 5],
            [5, 4],
            [5, 3],
            [5, 2]
        ]

        new_head = [5, 1]  # This would cause collision

        self.assertTrue(new_head in snake[1:])

if __name__ == "__main__":
    unittest.main(argv=[''], exit=False)
```

