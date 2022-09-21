import 'package:flutter/material.dart';
import 'package:notebook/components/list_item.dart';
import 'package:notebook/models/note.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/dashboard/note/create.dart';
import 'package:notebook/screens/dashboard/note/routes.dart';
import 'package:provider/provider.dart';

class NoteList extends StatefulWidget {
  const NoteList({Key? key}) : super(key: key);

  @override
  State<NoteList> createState() => _NoteListState();
}

class _NoteListState extends State<NoteList> {
  NoteConnection? _notes;
  dynamic _error;

  @override
  void initState() {
    _reload();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return _error == null
        ? _notes == null
            ? const CircularProgressIndicator()
            : ListView(children: [
                ...(_notes?.edges
                        .map((edge) => edge.node)
                        .map((note) => ListTile(
                              title: Text(note.title),
                              trailing: Text((note.updatedAt ?? note.createdAt)
                                  .toString()),
                              onTap: () {
                                Navigator.of(context).pushNamed(routeItem,
                                    arguments: RouteItemArguments(note.id));
                              },
                            ))
                        .toList() ??
                    []),
                const Divider(),
                Consumer<Client>(
                  builder: (context, client, child) => ListItem(
                      title: const Text('Create...'),
                      onTap: () async {
                        final note =
                            await Navigator.of(context, rootNavigator: true)
                                .push<CreateNoteArgs>(MaterialPageRoute(
                          builder: (context) => const NoteCreateScreen(),
                        ));
                        if (note != null) {
                          await client.createNote(note.title, note.content);
                          _reload();
                        }
                      }),
                )
              ])
        : Text(_error.toString());
  }

  _reload() {
    final client = Provider.of<Client?>(context, listen: false);
    if (client != null) {
      client.listNotes().then((res) {
        if (mounted) {
          setState(() {
            _notes = res;
          });
        }
      }).catchError((e) {
        setState(() {
          _error = e;
        });
      });
    }
  }
}
