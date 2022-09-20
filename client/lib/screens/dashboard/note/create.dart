import 'package:flutter/material.dart';
import 'package:notebook/components/layout.dart';

class NoteCreateScreen extends StatefulWidget {
  const NoteCreateScreen({Key? key}) : super(key: key);

  @override
  State<NoteCreateScreen> createState() => _NoteCreateScreenState();
}

class _NoteCreateScreenState extends State<NoteCreateScreen> {
  final _formKey = GlobalKey<FormState>();
  @override
  Widget build(BuildContext context) {
    return Layout(
      title: 'Create Note',
      body: Form(
          key: _formKey,
          child: Column(
            children: [
              TextFormField(
                decoration: InputDecoration(hintText: 'Title'),
              )
            ],
          )),
    );
  }
}
