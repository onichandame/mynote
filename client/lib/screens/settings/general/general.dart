import 'package:flutter/material.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/components/list_item.dart';
import 'package:notebook/components/text_input_screen.dart';
import 'package:notebook/providers/client.dart';
import 'package:provider/provider.dart';

class GeneralSettingsScreen extends StatelessWidget {
  const GeneralSettingsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Layout(
        title: 'General Settings',
        body: Consumer<Client?>(
            builder: (context, client, _) => client == null
                ? const CircularProgressIndicator()
                : ListView(
                    children: [
                      ListItem(
                          title: const Text('Backend URL'),
                          value: Text(
                            client.url,
                            style: Theme.of(context).textTheme.caption,
                          ),
                          onTap: () async {
                            final url = await Navigator.of(context)
                                .push(MaterialPageRoute(
                              builder: (context) => TextInputScreen(
                                  title: 'Backend URL',
                                  label: 'Enter new URL',
                                  initialValue: client.url),
                            ));
                            if (await showDialog<bool>(
                                    context: context,
                                    builder: (context) => AlertDialog(
                                          content: const Text(
                                              'Changing the backend url will lose all unsaved data. Are you sure?'),
                                          actions: [
                                            TextButton(
                                                onPressed: () {
                                                  Navigator.of(context).pop();
                                                },
                                                child: const Text('No')),
                                            TextButton(
                                                onPressed: () {
                                                  Navigator.of(context)
                                                      .pop(true);
                                                },
                                                child: Text(
                                                  'Yes',
                                                  style: TextStyle(
                                                      color: Theme.of(context)
                                                          .colorScheme
                                                          .error),
                                                ))
                                          ],
                                        )) ??
                                false) client.url = url;
                          })
                    ],
                  )));
  }
}
