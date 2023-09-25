extends Node2D

var animation

func _ready():
	animation = get_node("AnimatedSprite")

func update_dto(dto):
	position = dto.pos
	rotation = dto.angle
	
	if dto.current_speed > 1.0:
		animation.animation = "walk"
	else:
		animation.animation = "idle"
