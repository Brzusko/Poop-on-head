extends LineEdit;
class_name NumberLineEdit;

## Class is used to filter LineEdit value and remove non digit chars.

var _regex: RegEx = RegEx.new();
const _regex_pattern: String = "^[0-9]+$";
var _value_cache: String = "";

func _ready() -> void:
	_regex.compile(_regex_pattern);
	text_changed.connect(_on_text_changed);
	
func _on_text_changed(new_text: String) -> void:
	if _regex.search(new_text) == null:
		var caret_column_cache: int = caret_column;
		text = _value_cache;
		caret_column = caret_column_cache;
		return;
		
	_value_cache = new_text;
	
