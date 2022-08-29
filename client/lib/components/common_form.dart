import 'dart:async';

import 'package:flutter/material.dart';

class CommonForm extends StatefulWidget {
  final List<CommonFormField> fields;
  final FutureOr Function(BuildContext, Map<String, dynamic>, bool mounted)
      onSubmit;
  final String buttonLabel;

  const CommonForm(
      {Key? key,
      required this.fields,
      required this.onSubmit,
      required this.buttonLabel})
      : super(key: key);

  @override
  State<CommonForm> createState() => _CommonFormState();
}

class _CommonFormState extends State<CommonForm> {
  final _formKey = GlobalKey<FormState>();
  final Map<String, dynamic> _values = {};
  bool _busy = false;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 256),
        child: Form(
          key: _formKey,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              ...widget.fields.map((field) {
                switch (field.type) {
                  case CommonFormFieldType.text:
                    return TextFormField(
                      decoration: InputDecoration(labelText: field.name),
                      validator: (value) {
                        if (!field.nullable && value == null) {
                          return '${field.name} cannot be empty';
                        }
                        if (field.min != null &&
                            value != null &&
                            value.length < field.min!) {
                          return '${field.name} must be at least ${field.min} characters long';
                        }
                        if (field.max != null &&
                            value != null &&
                            value.length > field.max!) {
                          return '${field.name} must be at most ${field.max} characters long';
                        }
                        return null;
                      },
                      onSaved: (value) {
                        _values[field.name] = value;
                      },
                    );
                }
              }),
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 16),
                child: ElevatedButton(
                    onPressed: _busy
                        ? null
                        : () {
                            if (_formKey.currentState!.validate()) {
                              _formKey.currentState!.save();
                              (() async {
                                try {
                                  await widget.onSubmit(
                                      context, _values, mounted);
                                } catch (e) {
                                  ScaffoldMessenger.of(context)
                                      .showSnackBar(SnackBar(
                                    content: Text(e.toString()),
                                  ));
                                } finally {
                                  setState(() {
                                    _busy = false;
                                  });
                                }
                              })();
                            }
                            setState(() {
                              _busy = true;
                            });
                          },
                    child: Text(widget.buttonLabel)),
              )
            ],
          ),
        ),
      ),
    );
  }
}

class CommonFormField {
  final String name;
  final String? label;
  final bool nullable;
  final CommonFormFieldType type;
  final int? min;
  final int? max;
  CommonFormField(
      {required this.name,
      required this.type,
      this.nullable = false,
      this.label,
      this.min,
      this.max});
}

enum CommonFormFieldType { text }
