extends Area2D

var wallScene = preload("res://scenes/Wall.tscn")
var outWallLeftScene = preload("res://scenes/out_wall_left.tscn")
var outWallRightScene = preload("res://scenes/out_wall_right.tscn")
var playerOneScene = preload("res://scenes/Player1.tscn")
var playerTwoScene = preload("res://scenes/Player2.tscn")
var ballScene = preload("res://scenes/Ball.tscn")

func _ready():
	add_child(wallScene.instance())
	add_child(outWallLeftScene.instance())
	add_child(outWallRightScene.instance())
	add_child(playerOneScene.instance())
	add_child(playerTwoScene.instance())
	add_child(ballScene.instance())
