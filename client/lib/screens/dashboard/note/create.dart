import 'package:flutter/material.dart';
import 'package:notebook/components/layout.dart';

class NoteCreateScreen extends StatefulWidget {
  const NoteCreateScreen({Key? key}) : super(key: key);

  @override
  State<NoteCreateScreen> createState() => _NoteCreateScreenState();
}

class _NoteCreateScreenState extends State<NoteCreateScreen> {
  final _formKey = GlobalKey<FormState>();
  String? _title;
  String? _content;
  @override
  Widget build(BuildContext context) {
    return Layout(
      title: 'Create Note',
      body: Form(
          key: _formKey,
          child: Column(
            children: [
              TextFormField(
                decoration: const InputDecoration(hintText: 'Title'),
                onFieldSubmitted: (_) => _submit(context),
                onSaved: (value) {
                  _title = value;
                },
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Title cannot be empty';
                  }
                  return null;
                },
              ),
              TextFormField(
                decoration: const InputDecoration(hintText: 'Content'),
                maxLines: 6,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Content cannot be empty';
                  }
                  return null;
                },
                onSaved: (value) {
                  _content = value;
                },
              ),
              ElevatedButton(
                  onPressed: () => _submit(context),
                  child: const Text('Create'))
            ],
          )),
    );
  }

  _submit(BuildContext context) {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
    }
    Navigator.of(context).pop(CreateNoteArgs(_title!, _content!));
  }
}

class CreateNoteArgs {
  final String title;
  final String content;
  CreateNoteArgs(this.title, this.content);
}
