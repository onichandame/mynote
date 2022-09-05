import 'package:flutter/material.dart';
import 'package:flutter/src/foundation/key.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/components/list_item.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class BackendSettingsScreen extends StatelessWidget {
  const BackendSettingsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Layout(
        title: 'Backend Settings',
        body: Consumer<Client?>(
            builder: (context, client, _) => client == null
                ? const CircularProgressIndicator()
                : ListView(
                    children: [
                      ListItem(
                          title: const Text('URL'),
                          value: Text(
                            client.url,
                            style: Theme.of(context).textTheme.caption,
                          ),
                          onTap: () {
                            Navigator.of(context)
                                .pushNamed(routeSettingsBackendURL);
                          })
                    ],
                  )));
  }
}
