import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:notebook/components/avatar.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/components/list_item.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class ProfileScreen extends StatelessWidget {
  const ProfileScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Layout(
      title: 'Profile',
      body: Consumer<CurrentUser?>(
        builder: (context, currentUser, child) => currentUser?.user == null
            ? const CircularProgressIndicator()
            : ListView(children: [
                ListItem(
                    title: const Text('Avatar'),
                    value: const Avatar(),
                    onTap: () {
                      showDialog(
                        context: context,
                        builder: (context) => SimpleDialog(
                          children: [
                            SimpleDialogOption(
                              child: const Text('From Gallery'),
                              onPressed: () async {
                                final image = await ImagePicker()
                                    .pickImage(source: ImageSource.gallery);
                                print(image?.readAsBytes());
                              },
                            ),
                            SimpleDialogOption(
                              child: const Text('From URL'),
                              onPressed: () {
                                Navigator.of(context).popAndPushNamed(
                                    routeSettingsProfileAvatarUrl);
                              },
                            )
                          ],
                        ),
                      );
                    }),
                ListItem(
                    title: const Text('Username'),
                    value: Text(currentUser!.user!.name),
                    onTap: () {
                      Navigator.of(context)
                          .pushNamed(routeSettingsProfileUsername);
                    })
              ]),
      ),
    );
  }
}
