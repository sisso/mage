extends Node2D

var label

func _ready():
	label = get_node("description_label")

func update_dto(player_dto):
	var fmt = "HP: {0}/{1}\nMana: {2}/{3}\nCasting: {4}\nCalm down: {5}\nScore: {6}/{7}\nLevel: {8}"
	var buffer = fmt.format([
		stepify(player_dto.critter.hp, 0.01),
		stepify(player_dto.critter.max_hp, 0.01),
		stepify(player_dto.caster.mana, 0.01),
		stepify(player_dto.caster.max_mana, 0.01),
		stepify(player_dto.caster.casting, 0.01),
		stepify(player_dto.caster.calm_down, 0.01),
		player_dto.score,
		player_dto.score_next_level,
		player_dto.level,
	])

	label.text = buffer
