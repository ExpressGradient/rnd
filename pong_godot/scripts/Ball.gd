extends KinematicBody2D

var velocity = Vector2(-5, 2)
var collision

# 1234 - normal wall
# 1237 - left wall
# 1239 - right wall
# 1240 - player 1
# 1243 - player 2

func _physics_process(_delta):
	collision = move_and_collide(velocity)
	if collision:
		var collider_id = collision.collider_id
		var label_node = get_node("../").get_child(1)
		var scoreText = "{playerOneScore}-{playerTwoScore}"
		var playerOneScore = int(label_node.text.split("-")[0])
		var playerTwoScore = int(label_node.text.split("-")[1])
		
		if collider_id == 1237:
			print("Player1 loses")
			playerTwoScore += 1
		if collider_id == 1239:
			print("Player2 loses")
			playerOneScore += 1
			
		label_node.text = scoreText.format({
			"playerOneScore": playerOneScore, 
			"playerTwoScore": playerTwoScore
		})
		
		if collider_id == 1234:
			velocity.y = -velocity.y
		else:
			velocity.x = -velocity.x

		move_and_collide(velocity)
