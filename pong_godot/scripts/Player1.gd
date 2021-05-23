extends KinematicBody2D

func _process(delta):
	var velocity = Vector2(0, 0)
	var basic_velocity = 10000
	if Input.is_key_pressed(87):
		velocity.y -= delta * basic_velocity
	if Input.is_key_pressed(83):
		velocity.y += delta * basic_velocity
		
#	velocity.y = min(20, velocity.y)

	move_and_slide(velocity)
