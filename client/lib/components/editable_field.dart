import 'dart:async';

import 'package:flutter/material.dart';

class EditableField extends StatefulWidget {
  final String label;
  final String value;
  final FutureOr Function(BuildContext context, String value, bool mounted)
      onSubmit;

  const EditableField({
    Key? key,
    required this.label,
    required this.value,
    required this.onSubmit,
  }) : super(key: key);

  @override
  State<EditableField> createState() => _EditableFieldState();
}

class _EditableFieldState extends State<EditableField> {
  bool _isEditing = false;

  @override
  Widget build(BuildContext context) {
    return _isEditing
        ? TextField(
            decoration: InputDecoration(labelText: widget.label),
            autofocus: true,
            onSubmitted: (value) async {
              try {
                await widget.onSubmit(context, value, mounted);
              } catch (e) {
                if (mounted) {
                  ScaffoldMessenger.of(context)
                      .showSnackBar(SnackBar(content: Text(e.toString())));
                }
              } finally {
                if (mounted) {
                  setState(() {
                    _isEditing = false;
                  });
                }
              }
            },
          )
        : InkWell(
            child: Text(widget.value),
            onTap: () {
              setState(() {
                _isEditing = true;
              });
            },
          );
  }
}
