import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:notebook/models/note.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/dashboard/layout.dart';
import 'package:notebook/screens/dashboard/note/main.dart';
import 'package:provider/provider.dart';

class NoteDetail extends StatefulWidget {
  final int id;
  const NoteDetail({
    Key? key,
    required this.id,
  }) : super(key: key);

  @override
  State<NoteDetail> createState() => _NoteDetailState();
}

class _NoteDetailState extends State<NoteDetail> {
  Note? _note;
  dynamic _error;

  @override
  void initState() {
    _reload();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return DashboardLayout(
      title: _note?.title ?? noteNavigation.title,
      index: noteNavigationIndex,
      body: Center(
        child: _error == null
            ? _note == null
                ? const CircularProgressIndicator()
                : Column(children: [
                    Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: [
                        ElevatedButton(
                            onPressed: () {}, child: const Text('Edit')),
                        ElevatedButton(
                            style: ElevatedButton.styleFrom(
                                primary: Theme.of(context).colorScheme.error),
                            onPressed: () {},
                            child: const Text('Delete'))
                      ],
                    ),
                    Text(
                      _note!.title,
                      style: Theme.of(context).textTheme.titleLarge,
                    ),
                    MarkdownBody(
                      data: _note!.content,
                    )
                  ])
            : Text(_error.toString()),
      ),
    );
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
      }).catchError((e) {
        if (mounted) {
          setState(() {
            _error = e;
          });
        }
      });
    }
  }
}
