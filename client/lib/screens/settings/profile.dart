import 'package:flutter/material.dart';
import 'package:flutter/src/foundation/key.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:notebook/components/avatar.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:provider/provider.dart';

class ProfileScreen extends StatelessWidget {
  const ProfileScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<CurrentUser?>(
        builder: (context, currentUser, _) => Layout(
              title: 'Profile',
              body: Center(
                child: currentUser?.user == null
                    ? const CircularProgressIndicator()
                    : Padding(
                        padding: const EdgeInsets.only(top: 16),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.center,
                          children: [
                            const Avatar(),
                            Text(
                              currentUser!.user!.name,
                              style: Theme.of(context).textTheme.headline3,
                            )
                          ],
                        ),
                      ),
              ),
            ));
  }
}
