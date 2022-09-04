import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/src/foundation/key.dart';
import 'package:flutter/src/widgets/framework.dart';

class TextInput extends StatelessWidget {
  final String? label;
  final FutureOr Function(String) onChanged;
  final String? initialValue;
  const TextInput(
      {Key? key, this.label, this.initialValue, required this.onChanged})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 256),
      child: TextFormField(
        decoration: label == null ? null : InputDecoration(labelText: label),
        initialValue: initialValue,
        onChanged: onChanged,
      ),
    );
  }
}
