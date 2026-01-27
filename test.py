from pokr import PySettings
from pokr import PyGame
from pokr import PyAction

settings = PySettings(3, 1000)
game = PyGame(settings)
print(game.current_seat())
action = PyAction.new_fold()
game.play_turn(action)
print(game.current_seat())
