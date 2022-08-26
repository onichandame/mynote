import 'package:flutter/material.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'client.dart';

class Global extends StatelessWidget {
  final Widget? child;
  const Global({Key? key, this.child}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        FutureProvider(
          create: (context) => SharedPreferences.getInstance(),
          initialData: null,
        ),
        ChangeNotifierProxyProvider<SharedPreferences?, Client?>(
          create: (context) => null,
          update: (context, sharedPref, client) =>
              sharedPref == null ? null : Client(sharedPref),
        ),
        ChangeNotifierProxyProvider<Client?, CurrentUser?>(
            create: (_) => null,
            update: (context, client, _) =>
                client == null ? null : CurrentUser(client))
      ],
      child: child,
    );
  }
}
