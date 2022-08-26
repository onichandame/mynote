import 'package:flutter/material.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:provider/provider.dart';

class Avatar extends StatelessWidget {
  const Avatar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<CurrentUser?>(
        builder: (context, currentUser, child) => IconButton(
            onPressed: () {
              Scaffold.of(context).openEndDrawer();
            },
            icon: currentUser?.user == null
                ? const Icon(Icons.account_circle)
                : currentUser!.user?.avatar == null
                    ? Text(currentUser.user!.name
                        .trim()
                        .split(' ')
                        .map((v) => v[0])
                        .take(2)
                        .map((v) => v.toUpperCase())
                        .join())
                    : Image.network(currentUser.user!.avatar!)));
  }
}
