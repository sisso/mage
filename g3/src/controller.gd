extends Node2D

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var api
var ui

var idmap = {}

# Called when the node enters the scene tree for the first time.
func _ready():
	api = get_node("../api")
	print("found api %s" % api)
	
	ui = get_node("../arena_ui")
	print("found ui %s" % ui)

	api.start_scenery(get_viewport().get_visible_rect().size)

# Called every frame. 'delta' is the elapsed time since the previous frame.
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
	
