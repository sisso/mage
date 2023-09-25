extends Node2D


var animation


func _ready():
	animation = get_node("AnimatedSprite")

func update_dto(p_dto):
	position = p_dto.obj.pos
	rotation = p_dto.obj.angle
	
	if p_dto.obj.current_speed > 1.0:
		animation.animation = "walk"
	else:
		animation.animation = "idle"
