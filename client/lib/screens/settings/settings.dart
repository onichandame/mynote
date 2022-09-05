import 'package:flutter/material.dart';
import 'package:flutter/src/foundation/key.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/components/list_item.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  @override
  Widget build(BuildContext context) {
    return Consumer<Client?>(
        builder: (context, client, _) => Layout(
            title: 'Settings',
            body: Center(
              child: ListView(
                children: [
                  ListItem(
                    title: const Text('Profile'),
                    onTap: () {
                      Navigator.of(context).pushNamed(routeSettingsProfile);
                    },
                  ),
                  ListItem(
                    title: const Text('Backend'),
                    onTap: () {
                      Navigator.of(context).pushNamed(routeSettingsBackend);
                    },
                  )
                ],
              ),
            )));
  }
}
