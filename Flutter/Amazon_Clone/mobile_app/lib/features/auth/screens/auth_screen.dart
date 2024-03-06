import 'package:amazon_clone/common/widgets/custom_button.dart';
import 'package:amazon_clone/common/widgets/custom_textfield.dart';
import 'package:amazon_clone/constants/global_variables.dart';
import 'package:flutter/material.dart';

enum Auth {
  signUp,
  signIn,
}

class AuthScreen extends StatefulWidget {
  static const String routeName = "auth_screen";
  const AuthScreen({super.key});

  @override
  State<AuthScreen> createState() => _AuthScreenState();
}

class _AuthScreenState extends State<AuthScreen> {
  Auth _auth = Auth.signUp;

  final _signUpFormKey = GlobalKey<FormState>();
  final _signInFormKey = GlobalKey<FormState>();

  final TextEditingController _emailController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final TextEditingController _nameController = TextEditingController();

  @override
  void dispose() {
    super.dispose();
    _emailController.dispose();
    _passwordController.dispose();
    _nameController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: GlobalVariables.backgroundColor,
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(10),
          child: Column(children: [
            const Text("Welcome", style: TextStyle(fontSize: 22, fontWeight: FontWeight.w500)),
            if (_auth == Auth.signUp)
              Container(
                padding: const EdgeInsets.all(10),
                child: Form(
                  key: _signUpFormKey,
                  child: Column(
                    children: [
                      CustomTextField(controller: _nameController, hintText: "Name"),
                      const SizedBox(height: 10),
                      CustomTextField(controller: _emailController, hintText: "Email"),
                      const SizedBox(height: 10),
                      CustomTextField(controller: _passwordController, hintText: "Password"),
                      const SizedBox(height: 20),
                      CustomButton(text: "Sign Up", onTap: () {})
                    ],
                  ),
                ),
              ),
            if (_auth == Auth.signIn)
              Container(
                padding: const EdgeInsets.all(10),
                child: Form(
                  key: _signUpFormKey,
                  child: Column(
                    children: [
                      CustomTextField(controller: _emailController, hintText: "Email"),
                      const SizedBox(height: 10),
                      CustomTextField(controller: _passwordController, hintText: "Password"),
                      const SizedBox(height: 20),
                      CustomButton(text: "Sign In", onTap: () {})
                    ],
                  ),
                ),
              ),
            ListTile(
              title: const Text('Create acoount'),
              leading: Radio(
                  value: Auth.signUp,
                  groupValue: _auth,
                  onChanged: (Auth? val) {
                    setState(() {
                      _auth = val!;
                    });
                  }),
            ),
            ListTile(
              title: const Text('Sign-in'),
              leading: Radio(
                  value: Auth.signIn,
                  groupValue: _auth,
                  onChanged: (Auth? val) {
                    setState(() {
                      _auth = val!;
                    });
                  }),
            ),
          ]),
        ),
      ),
    );
  }
}
