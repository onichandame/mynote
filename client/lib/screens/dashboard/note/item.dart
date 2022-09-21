import 'package:flutter/material.dart';
import 'package:notebook/models/note.dart';
import 'package:notebook/providers/client.dart';
import 'package:provider/provider.dart';

class NoteItem extends StatefulWidget {
  final int id;
  const NoteItem({Key? key, required this.id}) : super(key: key);

  @override
  State<NoteItem> createState() => _NoteItemState();
}

class _NoteItemState extends State<NoteItem> {
  Note? _note;

  @override
  void initState() {
    _reload();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return _note == null
        ? const CircularProgressIndicator()
        : Column(children: [Text(_note!.title), Text(_note!.content)]);
  }

  _reload() {
    final client = Provider.of<Client?>(context, listen: false);
    if (client != null) {
      client.findNote(widget.id).then((res) {
        if (mounted) {
          setState(() {
            _note = res;
          });
        }
      });
    }
  }
}
