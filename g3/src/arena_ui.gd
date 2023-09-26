extends Node2D

var label

func _ready():
	label = get_node("description_label")

func update_dto(player_dto):
	var fmt = "Mana: {0}/{1}\nCasting: {2}\nCalm down: {3}\nScore: {4}"
	var caster = player_dto.caster
	var buffer = fmt.format([
		stepify(caster.mana, 0.01),
		stepify(caster.max_mana, 0.01),
		stepify(caster.casting, 0.01),
		stepify(caster.calm_down, 0.01),
		stepify(player_dto.score, 0.01),
	])

	label.text = buffer
