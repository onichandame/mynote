import 'package:flutter/material.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:provider/provider.dart';

class Avatar extends StatelessWidget {
  final void Function(BuildContext context)? onPressed;
  const Avatar({Key? key, this.onPressed}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<CurrentUser?>(
        builder: (context, currentUser, child) => CircleAvatar(
              child: IconButton(
                  onPressed: onPressed == null
                      ? null
                      : () {
                          onPressed!(context);
                        },
                  icon: currentUser?.user == null
                      ? const Icon(Icons.account_circle)
                      : currentUser!.user?.avatar == null
                          ? Text(
                              currentUser.user!.name
                                  .trim()
                                  .split(' ')
                                  .map((v) => v[0])
                                  .take(2)
                                  .map((v) => v.toUpperCase())
                                  .join(),
                              style:
                                  Theme.of(context).appBarTheme.titleTextStyle,
                            )
                          : Image.network(currentUser.user!.avatar!)),
            ));
  }
}
