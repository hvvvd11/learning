import 'package:flutter/material.dart';

class BannerWidget extends StatelessWidget {
  const BannerWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(top: 5, left: 15, right: 15),
      child: Container(
        height: 140,
        width: double.infinity - 15,
        decoration: BoxDecoration(
          color: Colors.yellow,
          borderRadius: BorderRadius.circular(15),
        ),
        child: PageView(
          children: const [
            Text("Slide 1"),
            Text("Slide 2"),
            Text("Slide 3"),
          ],
        ),
      ),
    );
  }
}
