extends Node2D

signal on_upgrade_button_pressed(code)

onready var label = $DescriptionLabel
onready var upgrade_buttons = $UpgradeContainer

func update_dto(player_dto):
	var fmt = "HP: {0}/{1}\nMana: {2}/{3}\nCasting: {4}\nCalm down: {5}\nScore: {6}/{7}\nLevel: {8}\nSkill: {9}"
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
		player_dto.free_skill_points
	])
	label.text = buffer
	
	upgrade_buttons.visible = player_dto.free_skill_points > 0;


func _on_upgrade_button_pressed(code):
	emit_signal("on_upgrade_button_pressed", code)
