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

