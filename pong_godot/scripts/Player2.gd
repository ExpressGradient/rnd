extends KinematicBody2D

func _process(delta):
	var velocity = Vector2(0, 0)
	var basic_velocity = 10000
	if Input.is_action_pressed("ui_up"):
		velocity.y -= delta * basic_velocity
	if Input.is_action_pressed("ui_down"):
		velocity.y += delta * basic_velocity
		
	move_and_slide(velocity)
