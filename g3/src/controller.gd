extends Node2D

onready var api = $"../api"
onready var ui = $"../arena_ui"

var idmap = {}

var request_upgrade = ""

func _ready():
	ui.connect("on_upgrade_button_pressed", self, "_on_click_skill_upgrade")
	
	api.start_scenery(get_viewport().get_visible_rect().size)

func _process(delta):
	var gi = api.new_run_update_input()
	gi.total_time = Time.get_ticks_msec()
	gi.delta_time = delta
	
	if Input.is_action_pressed("ui_right"):
		gi.input.x += 1
	if Input.is_action_pressed("ui_left"):
		gi.input.x -= 1
	if Input.is_action_pressed("ui_down"):
		gi.input.y += 1
	if Input.is_action_pressed("ui_up"):
		gi.input.y -= 1

	if Input.is_mouse_button_pressed(1):
		gi.mouse_press  = true

	gi.mouse_pos = get_viewport().get_mouse_position()
	if request_upgrade != "":
		print("setting requesting upgrade ", request_upgrade)
		gi.request_upgrade = request_upgrade
		request_upgrade = ""

	# print("running update ", gi)
	var output = api.run_update(gi)
	# print("receive ", output)

	# update player
	var player = get_node("../objects/player")
	player.update_dto(output.player)

	# update player ui
	ui.update_dto(output.player)

	# process events
	for id in output.removed:
		print("removing ", id)
		idmap[id].queue_free()
		idmap.erase(id)
		
	for obj in output.added:
		print("added ", obj)
		if obj.model == "magic_missile":
			var node = load("res://scenes/missile.tscn").instance()
			get_node("../objects").add_child(node)
			node.update_dto(obj)
			idmap[obj.id] = node
		elif obj.model == "enemy_1":
			var node = load("res://scenes/enemy_1.tscn").instance()
			get_node("../objects").add_child(node)
			node.update_dto(obj)
			idmap[obj.id] = node
		else:
			print("invalid model ", obj)

	for obj in output.objects:
		if idmap.has(obj.id):
			idmap[obj.id].update_dto(obj)
		else:
			print("invalid obj ", obj)
	

func _on_click_skill_upgrade(code):
	request_upgrade = code
