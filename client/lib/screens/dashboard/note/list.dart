import 'package:flutter/material.dart';
import 'package:notebook/components/list_item.dart';
import 'package:notebook/models/note.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/dashboard/note/create.dart';
import 'package:provider/provider.dart';

class NoteList extends StatefulWidget {
  const NoteList({Key? key}) : super(key: key);

  @override
  State<NoteList> createState() => _NoteListState();
}

class _NoteListState extends State<NoteList> {
  final List<Note> _notes = [];
  @override
  Widget build(BuildContext context) {
    return ListView(children: [
      ...(_notes
          .map((note) => ListTile(
                title: Text(note.title),
              ))
          .toList()),
      Consumer<Client>(
        builder: (context, client, child) => ListItem(
            title: const Text('Create...'),
            onTap: () async {
              final note = await Navigator.of(context).push(MaterialPageRoute(
                builder: (context) => const NoteCreateScreen(),
              ));
              if (note != null) {
                await client.createNote(note.title, note.content);
              }
            }),
      )
    ]);
  }
}
