```python
"""
# Snake Game Development Report

## Requirements Engineering

### Functional Requirements
1. The snake must move continuously in the direction of user input, with the head advancing one cell per update cycle.
2. The snake must grow by one cell each time it consumes food (represented by an 'O' character).
3. The game must end when the snake collides with itself or the boundaries of the terminal window.

### Non-functional Requirements
1. The game must maintain a consistent frame rate of 10 updates per second for smooth gameplay.
2. The game must handle terminal input efficiently without blocking or lagging during gameplay.

## Deployment

To run the Snake game:
1. Ensure Python 3.x is installed on your system
2. Run the script directly in a terminal: `python snake_game.py`
3. The game requires no external dependencies beyond the standard Python library
4. The game is designed to work in any terminal that supports curses functionality

## Codebase Support

Potential Bugs:
1. Terminal state corruption: If the program exits unexpectedly, the terminal may remain in curses mode. This can be avoided by properly handling exceptions and calling `curses.endwin()` in a finally block.
2. Input buffer overflow: Rapid key presses can cause input lag or missed inputs. This can be mitigated by using `curses.nodelay()` and implementing proper input queue management.

## Documentation

### Controls
- Arrow keys: Change snake direction
- 'q' or 'Q': Quit the game
- 'r' or 'R': Restart the game after game over

### Game Mechanics
- Snake grows when it eats food
- Game ends on collision with walls or self
- Score increases with each food consumed
"""

import curses
import random
import time
import sys

class SnakeGame:
    def __init__(self, stdscr):
        self.stdscr = stdscr
        self.height, self.width = stdscr.getmaxyx()
        self.snake = [(self.height//2, self.width//2)]
        self.direction = curses.KEY_RIGHT
        self.food = None
        self.score = 0
        self.game_over = False
        self.setup_game()
        
    def setup_game(self):
        """Initialize game state and place first food"""
        self.stdscr.nodelay(True)
        self.stdscr.timeout(100)  # 100ms timeout for getch()
        self.generate_food()
        
    def generate_food(self):
        """Generate food at random location not occupied by snake"""
        while True:
            self.food = (random.randint(1, self.height-2), random.randint(1, self.width-2))
            if self.food not in self.snake:
                break
                
    def draw(self):
        """Draw the game state to the screen"""
        self.stdscr.clear()
        
        # Draw border
        for y in range(self.height):
            for x in range(self.width):
                if y == 0 or y == self.height-1 or x == 0 or x == self.width-1:
                    self.stdscr.addch(y, x, '#')
        
        # Draw snake
        for i, (y, x) in enumerate(self.snake):
            char = 'O' if i == 0 else 'o'  # Head is different character
            self.stdscr.addch(y, x, char)
            
        # Draw food
        if self.food:
            self.stdscr.addch(self.food[0], self.food[1], 'O')
            
        # Draw score
        self.stdscr.addstr(0, 2, f"Score: {self.score}")
        
        # Draw game over message
        if self.game_over:
            game_over_msg = "GAME OVER! Press 'R' to restart or 'Q' to quit"
            self.stdscr.addstr(self.height//2, (self.width-len(game_over_msg))//2, game_over_msg)
            
        self.stdscr.refresh()
        
    def update(self):
        """Update game state"""
        if self.game_over:
            return
            
        # Calculate new head position
        head_y, head_x = self.snake[0]
        if self.direction == curses.KEY_UP:
            new_head = (head_y - 1, head_x)
        elif self.direction == curses.KEY_DOWN:
            new_head = (head_y + 1, head_x)
        elif self.direction == curses.KEY_LEFT:
            new_head = (head_y, head_x - 1)
        elif self.direction == curses.KEY_RIGHT:
            new_head = (head_y, head_x + 1)
        else:
            new_head = (head_y, head_x)
            
        # Check for collisions with walls
        if (new_head[0] <= 0 or new_head[0] >= self.height-1 or 
            new_head[1] <= 0 or new_head[1] >= self.width-1):
            self.game_over = True
            return
            
        # Check for collisions with self
        if new_head in self.snake:
            self.game_over = True
            return
            
        # Add new head
        self.snake.insert(0, new_head)
        
        # Check if food is eaten
        if new_head == self.food:
            self.score += 10
            self.generate_food()
        else:
            # Remove tail if no food eaten
            self.snake.pop()
            
    def handle_input(self):
        """Handle user input"""
        key = self.stdscr.getch()
        
        # Handle quit
        if key in [ord('q'), ord('Q')]:
            return False
            
        # Handle restart
        if key in [ord('r'), ord('R')] and self.game_over:
            self.restart_game()
            return True
            
        # Handle direction changes
        if key == curses.KEY_UP and self.direction != curses.KEY_DOWN:
            self.direction = curses.KEY_UP
        elif key == curses.KEY_DOWN and self.direction != curses.KEY_UP:
            self.direction = curses.KEY_DOWN
        elif key == curses.KEY_LEFT and self.direction != curses.KEY_RIGHT:
            self.direction = curses.KEY_LEFT
        elif key == curses.KEY_RIGHT and self.direction != curses.KEY_LEFT:
            self.direction = curses.KEY_RIGHT
            
        return True
        
    def restart_game(self):
        """Restart the game"""
        self.snake = [(self.height//2, self.width//2)]
        self.direction = curses.KEY_RIGHT
        self.score = 0
        self.game_over = False
        self.generate_food()

def main(stdscr):
    """Main game loop"""
    try:
        # Initialize curses
        curses.curs_set(0)  # Hide cursor
        stdscr.clear()
        stdscr.refresh()
        
        # Create game instance
        game = SnakeGame(stdscr)
        
        # Main game loop
        while True:
            # Handle input
            if not game.handle_input():
                break
                
            # Update game state
            game.update()
            
            # Draw game
            game.draw()
            
            # Small delay to control game speed
            time.sleep(0.1)
            
    except KeyboardInterrupt:
        pass
    except Exception as e:
        # Print error and exit cleanly
        stdscr.clear()
        stdscr.addstr(0, 0, f"Error: {str(e)}")
        stdscr.refresh()
        time.sleep(2)
    finally:
        # Always restore terminal state
        curses.endwin()

if __name__ == "__main__":
    curses.wrapper(main)
```

## Testing

```python
import unittest
from unittest.mock import Mock, patch
import curses

# Test cases for the Snake game logic
class TestSnakeGameLogic(unittest.TestCase):
    
    def test_snake_movement(self):
        """Test snake movement logic"""
        # Mock stdscr
        stdscr = Mock()
        game = SnakeGame(stdscr)
        
        # Test initial state
        self.assertEqual(game.snake[0], (25, 40))  # Default center position
        self.assertEqual(game.direction, curses.KEY_RIGHT)
        
        # Test movement update
        initial_snake = game.snake.copy()
        game.direction = curses.KEY_UP
        game.update()
        self.assertEqual(game.snake[0], (initial_snake[0][0] - 1, initial_snake[0][1]))
        
    def test_collision_detection(self):
        """Test collision detection logic"""
        # Mock stdscr
        stdscr = Mock()
        game = SnakeGame(stdscr)
        
        # Test self collision
        game.snake = [(10, 10), (10, 11), (10, 12)]
        game.direction = curses.KEY_LEFT
        game.update()
        self.assertTrue(game.game_over)
        
        # Test wall collision
        game = SnakeGame(stdscr)
        game.snake = [(1, 10)]
        game.direction = curses.KEY_LEFT
        game.update()
        self.assertTrue(game.game_over)

if __name__ == '__main__':
    unittest.main()
```

