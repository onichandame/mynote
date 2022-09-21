import 'package:flutter/material.dart';
import 'package:notebook/models/note.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/dashboard/note/routes.dart';
import 'package:provider/provider.dart';

class NoteItem extends StatefulWidget {
  const NoteItem({Key? key}) : super(key: key);

  @override
  State<NoteItem> createState() => _NoteItemState();
}

class _NoteItemState extends State<NoteItem> {
  Note? _note;

  @override
  void initState() {
    final client = Provider.of<Client?>(context);
    if (client != null) {
      client
          .findNote(
              (ModalRoute.of(context)!.settings.arguments as RouteItemArguments)
                  .id)
          .then((res) {
        setState(() {
          _note = res;
        });
      });
    }
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    final args =
        ModalRoute.of(context)!.settings.arguments as RouteItemArguments;
    return Column(children: []);
  }
}
