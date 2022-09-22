import 'package:flutter/material.dart';
import 'package:notebook/components/common_form.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class LoginScreen extends StatelessWidget {
  const LoginScreen({Key? key}) : super(key: key);
  @override
  Widget build(BuildContext context) {
    final client = Provider.of<Client?>(context);
    return Layout(
        title: 'Log In',
        body: CommonForm(
          fields: [
            CommonFormField(
                name: 'identity',
                label: 'Name/Email',
                type: CommonFormFieldType.text),
            CommonFormField(
                name: 'password',
                label: 'Password',
                type: CommonFormFieldType.text)
          ],
          buttonLabel: 'Log In',
          onSubmit: (context, values, mounted) async {
            final session = await client!.login(
                identity: values['identity'], password: values['password']);
            if (mounted) {
              ScaffoldMessenger.of(context)
                  .showSnackBar(const SnackBar(content: Text('logged in')));
              client.session = session;
              Navigator.of(context)
                  .pushNamedAndRemoveUntil(defaultRoute, (_) => false);
            }
          },
        ));
  }
}
