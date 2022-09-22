import 'package:flutter/material.dart';
import 'package:notebook/components/common_form.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class SignupScreen extends StatefulWidget {
  const SignupScreen({Key? key}) : super(key: key);

  @override
  State<SignupScreen> createState() => _SignupScreenState();
}

class _SignupScreenState extends State<SignupScreen> {
  @override
  Widget build(BuildContext context) {
    return Consumer<Client?>(
        builder: (context, client, _) => Layout(
            title: 'Sign Up',
            body: CommonForm(
              fields: [
                CommonFormField(
                  name: 'name',
                  type: CommonFormFieldType.text,
                ),
                CommonFormField(
                    name: 'password',
                    type: CommonFormFieldType.text,
                    min: 5,
                    max: 10)
              ],
              buttonLabel: 'Sign Up',
              onSubmit: (context, values, mounted) async {
                final session = await client!
                    .signup(name: values['name'], password: values['password']);
                if (mounted) {
                  client.session = session;
                  ScaffoldMessenger.of(context)
                      .showSnackBar(const SnackBar(content: Text('signed up')));
                  Navigator.of(context)
                      .pushNamedAndRemoveUntil(defaultRoute, (_) => false);
                }
              },
            )));
  }
}
