from pokr import PySettings
from pokr import PyGame
from pokr import PyAction

settings = PySettings(3, 1000, 10, 1000)

game = PyGame(settings)

while (not game.is_over()):
    action = PyAction.new_fold()
    game.play_turn(action)

print("game is over")