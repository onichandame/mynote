import 'package:flutter/material.dart';
import 'package:notebook/components/layout.dart';

class TextInputScreen extends StatefulWidget {
  final String title;
  final String label;
  final String? initialValue;
  const TextInputScreen(
      {Key? key, required this.title, required this.label, this.initialValue})
      : super(key: key);

  @override
  State<TextInputScreen> createState() => _TextInputScreenState();
}

class _TextInputScreenState extends State<TextInputScreen> {
  @override
  Widget build(BuildContext context) {
    return Layout(
      title: widget.title,
      body: TextFormField(
        decoration: InputDecoration(labelText: widget.label),
        initialValue: widget.initialValue,
        onFieldSubmitted: (value) async {
          if (mounted) Navigator.of(context).pop(value);
        },
      ),
    );
  }
}
