import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

class WelcomeTextWidget extends StatelessWidget {
  const WelcomeTextWidget({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.only(top: MediaQuery.of(context).padding.top + 10, left: 25, right: 25),
      child: Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
        const Text("Good day,\nLalala", style: TextStyle(fontSize: 16)),
        Container(
          child: SvgPicture.asset(
            'assets/icons/cart.svg',
            width: 25,
          ),
        )
      ]),
    );
  }
}
