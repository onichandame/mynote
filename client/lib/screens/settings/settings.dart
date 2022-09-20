import 'package:flutter/material.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/components/list_item.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/providers/current_user.dart';
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
    return Consumer2<Client?, CurrentUser?>(
        builder: (context, client, currentUser, _) => Layout(
            title: 'Settings',
            body: Center(
              child: ListView(
                children: [
                  ...(currentUser?.user == null
                      ? []
                      : [
                          ListItem(
                            title: const Text('Profile'),
                            onTap: () {
                              Navigator.of(context)
                                  .pushNamed(routeSettingsProfile);
                            },
                          ),
                          const Divider()
                        ]),
                  ListItem(
                    title: const Text('General'),
                    onTap: () {
                      Navigator.of(context).pushNamed(routeSettingsGeneral);
                    },
                  )
                ],
              ),
            )));
  }
}
