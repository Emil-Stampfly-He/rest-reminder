"""
Uncomment the code below for enabling plugin
"""

_SHOULD_IGNORE = 1

import tkinter as tk
from tkinter import messagebox
import random

class SnakeGame:
    def __init__(self, root):
        self.root = root
        self.root.title("Snake Game")
        self.root.resizable(False, False)

        # Game params
        self.GAME_WIDTH = 700
        self.GAME_HEIGHT = 700
        self.SPEED = 100
        self.SPACE_SIZE = 50
        self.BODY_PARTS = 3
        self.SNAKE_COLOR = "#00FF00"
        self.FOOD_COLOR = "#FF0000"
        self.BACKGROUND_COLOR = "#FFFFFF"

        self.direction = 'right'
        self.score = 0

        # UI
        self.label = tk.Label(self.root, text=f"Score: {self.score}", font=("Arial", 20))
        self.label.pack()

        self.canvas = tk.Canvas(self.root, bg=self.BACKGROUND_COLOR, width=self.GAME_WIDTH, height=self.GAME_HEIGHT)
        self.canvas.pack()

        # Initiating
        self.root.update()
        self.root.bind('<Up>', lambda event: self.update_direction('up'))
        self.root.bind('<Down>', lambda event: self.update_direction('down'))
        self.root.bind('<Left>', lambda event: self.update_direction('left'))
        self.root.bind('<Right>', lambda event: self.update_direction('right'))

        self.snake_position = []
        self.snake_squares = []
        self.food = None
        self.food_position = None

        self.start_game()

    def start_game(self):
        # Initialize snake
        self.snake_position = []
        self.snake_squares = []
        self.score = 0
        self.direction = 'right'
        self.label.config(text=f"Score: {self.score}")

        for i in range(self.BODY_PARTS):
            x = self.SPACE_SIZE * (self.BODY_PARTS - i)
            y = self.SPACE_SIZE
            self.snake_position.append([x, y])
            square = self.canvas.create_rectangle(x, y, x+self.SPACE_SIZE, y+self.SPACE_SIZE, fill=self.SNAKE_COLOR, tags="snake")
            self.snake_squares.append(square)

        self.spawn_food()
        self.move()

    def spawn_food(self):
        if self.food:
            self.canvas.delete(self.food)

        # Ensure food doesn't spawn on snake
        while True:
            x = random.randint(0, (self.GAME_WIDTH // self.SPACE_SIZE) - 1) * self.SPACE_SIZE
            y = random.randint(0, (self.GAME_HEIGHT // self.SPACE_SIZE) - 1) * self.SPACE_SIZE
            self.food_position = [x, y]
            if self.food_position not in self.snake_position:
                break

        self.food = self.canvas.create_oval(x, y, x+self.SPACE_SIZE, y+self.SPACE_SIZE, fill=self.FOOD_COLOR, tags="food")

    def move(self):
        head = self.snake_position[0].copy()

        if self.direction == 'up':
            head[1] -= self.SPACE_SIZE
        elif self.direction == 'down':
            head[1] += self.SPACE_SIZE
        elif self.direction == 'left':
            head[0] -= self.SPACE_SIZE
        elif self.direction == 'right':
            head[0] += self.SPACE_SIZE

        self.snake_position.insert(0, head)
        square = self.canvas.create_rectangle(head[0], head[1], head[0]+self.SPACE_SIZE, head[1]+self.SPACE_SIZE, fill=self.SNAKE_COLOR, tags="snake")
        self.snake_squares.insert(0, square)

        # Check if eating food
        if head == self.food_position:
            self.score += 1
            self.label.config(text=f"Score: {self.score}")
            self.spawn_food()
        else:
            self.snake_position.pop()
            self.canvas.delete(self.snake_squares.pop())

        # Check if game over
        if self.check_collisions():
            self.game_over()
        else:
            self.root.after(self.SPEED, self.move)

    def check_collisions(self):
        head = self.snake_position[0]

        # Run into walls
        if head[0] < 0 or head[0] >= self.GAME_WIDTH or head[1] < 0 or head[1] >= self.GAME_HEIGHT:
            return True

        # Run into itself
        for body_part in self.snake_position[1:]:
            if head == body_part:
                return True

        return False

    def game_over(self):
        self.canvas.delete("all")
        response = messagebox.askyesno("Game Over", f"Final Score: {self.score}\nWould you like to retry?")
        if response:
            self.canvas.delete("all")  # Clear canvas again for safety
            self.start_game()  # Restart the game
        else:
            self.root.destroy()  # Quit the game

    def update_direction(self, new_direction):
        if (new_direction == 'up' and self.direction != 'down') or \
           (new_direction == 'down' and self.direction != 'up') or \
           (new_direction == 'left' and self.direction != 'right') or \
           (new_direction == 'right' and self.direction != 'left'):
            self.direction = new_direction

def on_break_reminder(_context):
    root = tk.Tk()
    _game = SnakeGame(root)
    root.mainloop()

# For testing
if __name__ == "__main__":
    root = tk.Tk()
    game = SnakeGame(root)
    root.mainloop()