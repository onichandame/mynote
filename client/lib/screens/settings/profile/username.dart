import 'package:flutter/material.dart';
import 'package:flutter/src/foundation/key.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:provider/provider.dart';

class UsernameProfileSettingsScreen extends StatefulWidget {
  const UsernameProfileSettingsScreen({Key? key}) : super(key: key);

  @override
  State<UsernameProfileSettingsScreen> createState() =>
      _UsernameProfileSettingsScreenState();
}

class _UsernameProfileSettingsScreenState
    extends State<UsernameProfileSettingsScreen> {
  @override
  Widget build(BuildContext context) {
    return Layout(
        title: 'Username',
        body: Consumer<CurrentUser?>(
          builder: (context, currentUser, child) => currentUser?.user == null
              ? const CircularProgressIndicator()
              : TextFormField(
                  decoration:
                      const InputDecoration(labelText: 'Enter new username'),
                  initialValue: currentUser!.user!.name,
                  onFieldSubmitted: (value) async {
                    if (mounted) Navigator.of(context).pop(value);
                  },
                ),
        ));
  }
}
