import 'package:flutter/material.dart';
import 'package:flutter/src/foundation/key.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:provider/provider.dart';

class UrlAvatarProfileSettingsScreen extends StatefulWidget {
  const UrlAvatarProfileSettingsScreen({Key? key}) : super(key: key);

  @override
  State<UrlAvatarProfileSettingsScreen> createState() =>
      _UrlAvatarProfileSettingsScreenState();
}

class _UrlAvatarProfileSettingsScreenState
    extends State<UrlAvatarProfileSettingsScreen> {
  @override
  Widget build(BuildContext context) {
    return Layout(
        title: 'Avatar',
        body: Consumer<CurrentUser?>(
            builder: (context, currentUser, child) => currentUser?.user == null
                ? const CircularProgressIndicator()
                : TextFormField(
                    decoration: const InputDecoration(
                        labelText: 'Enter URL for new avatar'),
                    onFieldSubmitted: (value) async {
                      await currentUser!.updateAvatar(value);
                      if (mounted) Navigator.of(context).pop();
                    },
                  )));
  }
}
