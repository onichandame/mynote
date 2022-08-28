import 'package:flutter/material.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class Layout extends StatelessWidget {
  final Widget body;
  final String title;
  final Widget? bottomNavigationBar;

  const Layout(
      {Key? key,
      required this.title,
      required this.body,
      this.bottomNavigationBar})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: Navigator.canPop(context)
            ? IconButton(
                icon: const Icon(Icons.arrow_back),
                onPressed: () {
                  Navigator.pop(context);
                },
              )
            : null,
        title: Text(title),
        actions: const [Avatar()],
      ),
      endDrawer: const AvatarDrawer(),
      bottomNavigationBar: bottomNavigationBar,
      body: body,
    );
  }
}

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
                    ? Text(
                        currentUser.user!.name
                            .trim()
                            .split(' ')
                            .map((v) => v[0])
                            .take(2)
                            .map((v) => v.toUpperCase())
                            .join(),
                        style: Theme.of(context).appBarTheme.titleTextStyle,
                      )
                    : Image.network(currentUser.user!.avatar!)));
  }
}

class AvatarDrawer extends StatelessWidget {
  const AvatarDrawer({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer2<CurrentUser?, Client?>(
        builder: (context, currentUser, client, _) => Drawer(
              child: ListView(
                children: currentUser?.user == null
                    ? [
                        ListTile(
                          title: const Text('Log In'),
                          leading: const Icon(Icons.login),
                          onTap: () {
                            Navigator.pop(context);
                            Navigator.pushNamed(context, routeLogin);
                          },
                        ),
                        ListTile(
                          title: const Text('Sign Up'),
                          leading: const Icon(Icons.account_circle),
                          onTap: () {
                            Navigator.pop(context);
                            Navigator.pushNamed(context, routeSignup);
                          },
                        )
                      ]
                    : [
                        ListTile(
                          title: const Text('Log Out'),
                          leading: const Icon(Icons.logout),
                          onTap: () {
                            Navigator.pop(context);
                            client!.session = null;
                          },
                        )
                      ],
              ),
            ));
  }
}
