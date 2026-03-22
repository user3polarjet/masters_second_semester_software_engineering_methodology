import curses
from random import randint

# Constants
WIDTH = 60
HEIGHT = 20
TIMEOUT = 100
SNAKE_X = WIDTH // 4
SNAKE_Y = HEIGHT // 2
TAIL_LEN = 3
FOOD = None
SCORE = 0
DIRECTION = curses.KEY_RIGHT

# Initialize the screen
screen = curses.initscr()
curses.curs_set(0)
screen.timeout(TIMEOUT)

def add_food():
    global FOOD
    while FOOD is None:
        new_food = [randint(1, WIDTH - 2), randint(1, HEIGHT - 2)]
        if new_food not in snake:
            FOOD = new_food

def draw():
    screen.clear()
    screen.addstr(0, 0, f'Score: {SCORE} ')
    for body in snake:
        screen.addch(body[1], body[0], curses.ACS_CKBOARD)
    screen.addch(FOOD[1], FOOD[0], curses.ACS_DIAMOND)

def main():
    global SNAKE_X, SNAKE_Y, DIRECTION, SCORE, FOOD
    
    snake = [[SNAKE_X - i, SNAKE_Y] for i in range(TAIL_LEN)]
    add_food()
    
    while True:
        key = screen.getch()
        if key != -1:
            DIRECTION = key
        
        new_head = [snake[0][0], snake[0][1]]
        if DIRECTION == curses.KEY_RIGHT:
            new_head[0] += 1
        elif DIRECTION == curses.KEY_LEFT:
            new_head[0] -= 1
        elif DIRECTION == curses.KEY_UP:
            new_head[1] -= 1
        elif DIRECTION == curses.KEY_DOWN:
            new_head[1] += 1
        
        snake.insert(0, new_head)
        if snake[0] == FOOD:
            global SCORE
            SCORE += 1
            add_food()
            draw()
        else:
            snake.pop()
        draw()
        
if __name__ == '__main__':
    try:
        main()
    finally:
        curses.endwin()

