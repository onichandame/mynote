import 'package:flutter/material.dart';
import 'package:notebook/components/avatar.dart';
import 'package:notebook/components/editable_field.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:provider/provider.dart';

class ProfileScreen extends StatefulWidget {
  const ProfileScreen({Key? key}) : super(key: key);

  @override
  State<ProfileScreen> createState() => _ProfileScreenState();
}

class _ProfileScreenState extends State<ProfileScreen> {
  @override
  Widget build(BuildContext context) {
    return Layout(
        title: 'Profile',
        body: Center(
          child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 256),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Row(
                    children: [
                      Avatar(
                        onPressed: (context) {
                          // TODO: open edit dialog
                        },
                      ),
                      Consumer2<CurrentUser?, Client?>(
                          builder: (context, currentUser, client, _) =>
                              EditableField(
                                  label: 'Username',
                                  value: currentUser!.user!.name,
                                  onSubmit: (context, value, mounted) async {
                                    await client!.updateSelf(name: value);
                                    currentUser.user = await client.getUser();
                                  }))
                    ],
                  )
                ],
              )),
        ));
  }
}
