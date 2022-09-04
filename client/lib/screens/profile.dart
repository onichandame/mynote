import 'package:flutter/material.dart';
import 'package:notebook/components/avatar.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/components/text_input.dart';
import 'package:notebook/components/web_image_selector.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:provider/provider.dart';

class ProfileScreen extends StatefulWidget {
  const ProfileScreen({Key? key}) : super(key: key);

  @override
  State<ProfileScreen> createState() => _ProfileScreenState();
}

class _ProfileScreenState extends State<ProfileScreen> {
  bool _isEditing = false;
  String? _name;
  String? _avatar;
  String? _email;
  @override
  Widget build(BuildContext context) {
    return Layout(
        title: 'Profile',
        body: Center(
            child: Padding(
                padding: const EdgeInsets.only(top: 16),
                child: Consumer<CurrentUser?>(
                    builder: (context, currentUser, _) => currentUser?.user ==
                            null
                        ? const Text('loading')
                        : Column(
                            crossAxisAlignment: CrossAxisAlignment.center,
                            children: [
                              Wrap(
                                alignment: WrapAlignment.center,
                                crossAxisAlignment: WrapCrossAlignment.center,
                                spacing: 10,
                                children: [
                                  Avatar(
                                    onPressed: _isEditing
                                        ? (context) {
                                            showDialog(
                                                context: context,
                                                builder: (context) =>
                                                    WebImageSelector(
                                                        onSubmit: (value) {
                                                      _avatar = value;
                                                    }));
                                          }
                                        : null,
                                  ),
                                  _isEditing
                                      ? TextInput(
                                          label: 'Username',
                                          initialValue: currentUser!.user!.name,
                                          onChanged: (value) {
                                            _name = value;
                                          })
                                      : Text(currentUser!.user!.name),
                                ],
                              ),
                              if (currentUser.user!.email != null)
                                Wrap(
                                  children: [
                                    const Text('email:'),
                                    Text(currentUser.user!.email!)
                                  ],
                                ),
                              Padding(
                                padding: const EdgeInsets.only(top: 10),
                                child: Consumer<Client?>(
                                    builder: (context, client, _) => client ==
                                            null
                                        ? const Text('loading')
                                        : ElevatedButton(
                                            child: Text(
                                                _isEditing ? 'Update' : 'Edit'),
                                            onPressed: () {
                                              if (_isEditing) {
                                                _update(client, currentUser);
                                              }
                                              setState(() {
                                                _isEditing = !_isEditing;
                                              });
                                            },
                                          )),
                              )
                            ],
                          )))));
  }

  _update(Client client, CurrentUser currentUser) async {
    await client.updateSelf(name: _name, email: _email, avatar: _avatar);
    currentUser.reload();
  }
}
