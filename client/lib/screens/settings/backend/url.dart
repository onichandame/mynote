import 'package:flutter/material.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class UrlBackendSettingsScreen extends StatefulWidget {
  const UrlBackendSettingsScreen({Key? key}) : super(key: key);

  @override
  State<UrlBackendSettingsScreen> createState() =>
      _UrlBackendSettingsScreenState();
}

class _UrlBackendSettingsScreenState extends State<UrlBackendSettingsScreen> {
  @override
  Widget build(BuildContext context) {
    return Layout(
        title: 'Update Backend URL',
        body: Consumer<Client?>(
            builder: (context, client, _) => client == null
                ? const CircularProgressIndicator()
                : TextFormField(
                    decoration:
                        const InputDecoration(labelText: 'Enter new URL'),
                    initialValue: client.url,
                    onFieldSubmitted: (value) {
                      showDialog(
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
                                        client.url = value;
                                        Navigator.of(context)
                                            .pushNamedAndRemoveUntil(
                                                routeHome, (_) => false);
                                      },
                                      child: Text(
                                        'Yes',
                                        style: TextStyle(
                                            color: Theme.of(context)
                                                .colorScheme
                                                .error),
                                      ))
                                ],
                              ));
                    },
                  )));
  }
}
