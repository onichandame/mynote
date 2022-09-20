import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:notebook/components/avatar.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/components/list_item.dart';
import 'package:notebook/components/text_input_screen.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/providers/current_user.dart';
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
                    onTap: () async {
                      final url = await showDialog<String>(
                        context: context,
                        builder: (context) => SimpleDialog(
                          children: [ImageSource.camera, ImageSource.gallery]
                              .map((src) => Consumer<Client?>(
                                    builder: (context, client, child) =>
                                        client == null
                                            ? const CircularProgressIndicator()
                                            : SimpleDialogOption(
                                                child: Text('From ${src.name}'),
                                                onPressed: () async {
                                                  Navigator.of(context)
                                                      .pop(await () async {
                                                    final image =
                                                        await ImagePicker()
                                                            .pickImage(
                                                                source: src);
                                                    if (image != null) {
                                                      return await client
                                                          .uploadFile(image);
                                                    }
                                                  }());
                                                },
                                              ),
                                  ))
                              .toList(),
                        ),
                      );
                      if (url != null) await currentUser!.updateAvatar(url);
                    }),
                ListItem(
                    title: const Text('Username'),
                    value: Text(currentUser!.user!.name),
                    onTap: () async {
                      final name = await Navigator.of(context)
                          .push<String>(MaterialPageRoute(
                        builder: (context) => TextInputScreen(
                            title: 'Username',
                            initialValue: currentUser.user!.name,
                            label: 'Enter new username'),
                      ));
                      if (name != null) await currentUser.updateName(name);
                    }),
                ListItem(
                    title: const Text('Email'),
                    value: Text(currentUser.user!.email ?? ''),
                    onTap: () async {
                      final email = await Navigator.of(context)
                          .push<String>(MaterialPageRoute(
                        builder: (context) => TextInputScreen(
                            title: 'Email',
                            label: 'Enter new Email',
                            initialValue: currentUser.user!.email),
                      ));
                      if (email != null) await currentUser.updateEmail(email);
                    })
              ]),
      ),
    );
  }
}
