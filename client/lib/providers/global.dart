import 'package:flutter/material.dart';
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
        ChangeNotifierProvider(
          create: (context) => Client(
            Provider.of<SharedPreferences?>(context, listen: false),
          ),
        ),
        FutureProvider(
            create: (context) async {
              final client = Provider.of<Client>(context, listen: false);
              if (client.session == null) {
                return null;
              } else {
                try {
                  return await client.getUser();
                } catch (e) {
                  ScaffoldMessenger.of(context)
                      .showSnackBar(SnackBar(content: Text(e.toString())));
                  client.session = null;
                }
              }
            },
            initialData: null)
      ],
      child: child,
    );
  }
}
